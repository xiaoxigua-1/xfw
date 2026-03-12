local RuntimeState = {
	root = nil,
	nodes_by_id = {},
	dirty = {},
	ipc_handlers = {},
	global_states = {},
	current_node_id = nil,
}

local function mark_dirty(node_id, key, value)
	-- Notify Rust about state change
	if __xfw_notify_state_change then
		local json_value = nil
		if type(value) == "string" then
			json_value = '"' .. value .. '"'
		elseif type(value) == "number" then
			json_value = tostring(value)
		elseif type(value) == "boolean" then
			json_value = tostring(value)
		elseif type(value) == "table" then
			json_value = "{}"
		else
			json_value = "null"
		end
		__xfw_notify_state_change(key, json_value)
	end

	if node_id then
		table.insert(RuntimeState.dirty, { id = node_id, key = key, value = value })
	else
		table.insert(RuntimeState.dirty, { key = key, value = value })
	end
end

local function create_state_proxy(initial, path)
	local raw = initial or {}
	path = path or {}

	local function get_path_string()
		return table.concat(path, ".")
	end

	local function create_child_proxy(key, child_value)
		local child_path = { unpack(path) }
		table.insert(child_path, key)

		if type(child_value) == "table" and not getmetatable(child_value) then
			return create_state_proxy(child_value, child_path)
		end
		return child_value
	end

	local proxy = {}

	local mt = {
__index = function(_, key)
		if key == "__raw" then
			return raw
		elseif key == "__path" then
			return get_path_string()
		elseif key == "__is_state" then
			return true
		end

-- Register dependency when accessing state
local path_str = get_path_string()
if path_str ~= "" and __xfw_register_state and RuntimeState.current_node_id then
local full_path = path_str .. "." .. tostring(key)
__xfw_register_state(RuntimeState.current_node_id, full_path)
		end

			local value = raw[key]
			if type(value) == "table" and not getmetatable(value) then
				return create_child_proxy(key, value)
			end
			return value
		end,
		__newindex = function(_, key, value)
			if key == "__raw" or key == "__path" or key == "__is_state" then
				error("cannot modify internal state properties", 2)
			end
			local old_value = raw[key]
			raw[key] = value
			mark_dirty(nil, get_path_string() .. "." .. tostring(key), value)
		end,
		__pairs = function()
			return pairs(raw)
		end,
		__len = function()
			return #raw
		end,
	}

	setmetatable(proxy, mt)
	return proxy
end

local function wrap_node(raw, parent_proxy)
-- Set current node ID for state dependency tracking BEFORE processing
RuntimeState.current_node_id = raw.id

local children = {}
local node_id = raw.id

-- Register node with Rust and evaluate dynamic functions to track dependencies
if node_id and __xfw_register_state then
local function register_dependencies(obj, prefix)
prefix = prefix or ""
for key, value in pairs(obj) do
if key == "parent" or key == "children" or key == "__raw" or key == "__readonly_proxy" then
-- skip
elseif type(value) == "function" then
local path = prefix .. key
local ok, result = pcall(value)
if ok then
obj[key] = result
end
elseif type(value) == "table" and not getmetatable(value) then
local new_prefix = prefix .. key .. "."
register_dependencies(value, new_prefix)
end
end
end
register_dependencies(raw)
end

-- Clear current node ID after processing
RuntimeState.current_node_id = nil

	local function get_parent_proxy()
		if not parent_proxy then
			return nil
		end
		return parent_proxy.__readonly_proxy or parent_proxy
	end

	local proxy = {}
	local readonly_proxy = {}

	local function handle_read(_, key)
		if key == "parent" then
			return get_parent_proxy()
		elseif key == "children" then
			return children
		elseif key == "__raw" then
			return raw
		else
			return raw[key]
		end
	end

	local function handle_write(_, key, value)
		if key == "parent" then
			error("parent reference is read-only", 2)
		end
		raw[key] = value
		mark_dirty(raw.id, key, value)
	end

	setmetatable(proxy, {
		__index = handle_read,
		__newindex = handle_write,
		__pairs = function()
			return pairs(raw)
		end,
	})

	setmetatable(readonly_proxy, {
		__index = handle_read,
		__newindex = function()
			error("child nodes cannot mutate parent state", 2)
		end,
		__pairs = function()
			return pairs(raw)
		end,
	})

	proxy.__readonly_proxy = readonly_proxy

	if type(raw.children) == "table" then
		for index, child in ipairs(raw.children) do
			children[index] = wrap_node(child, proxy)
		end
	end

	if raw.id then
		RuntimeState.nodes_by_id[raw.id] = proxy
	end

	return proxy
end

local Runtime = {}

Runtime.UI = {}

function Runtime.UI.state(initial)
	local state = create_state_proxy(initial, { "global" })
	table.insert(RuntimeState.global_states, state)
	return state
end

function Runtime.UI.render(node)
	RuntimeState.nodes_by_id = {}
	RuntimeState.dirty = {}
	RuntimeState.root = wrap_node(node, nil)
end

function Runtime.UI.get_by_id(id)
	return RuntimeState.nodes_by_id[id]
end

function Runtime.UI.get_dirty()
	local dirty = RuntimeState.dirty
	RuntimeState.dirty = {}
	return dirty
end

function Runtime.UI.clear_dirty()
	RuntimeState.dirty = {}
end

Runtime.System = {}

function Runtime.System.execute(cmd)
	-- TODO: forward to Rust via IPC; shell out for now.
	os.execute(cmd)
end

Runtime.IPC = { handlers = {} }

function Runtime.IPC.on(event, handler)
	Runtime.IPC.handlers[event] = handler
end

function Runtime.IPC.emit(event, payload)
	local handler = Runtime.IPC.handlers[event]
	if handler then
		handler(payload)
	end
end

Runtime.state = RuntimeState
_G.__xfw_state = RuntimeState

return Runtime