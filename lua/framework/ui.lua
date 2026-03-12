local next_node_id = 0

local function normalize_children(props)
	props.children = props.children or {}
	return props
end

local function define(kind)
	return function(props)
		props = props or {}
		props.kind = kind
		props.id = props.id or ("n" .. next_node_id)
		next_node_id = next_node_id + 1
		return normalize_children(props)
	end
end

local constructors = {}

constructors.View = define("view")
constructors.Text = define("text")
constructors.Image = define("image")
constructors.Window = define("window")
constructors.Button = define("button")
constructors.Row = define("row")
constructors.Column = define("column")

return constructors
