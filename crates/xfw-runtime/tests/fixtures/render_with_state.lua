local Store = UI.state({
    items = {
        { id = "item1", color = "#ff9a3c" },
        { id = "item2", color = "#3a7ca5" },
    },
})

local function ItemWidget(props)
    print("[LUA] ItemWidget called, props.color type=" .. type(props.color))
    return View({
        style = {
            bg_color = props.color(),
            width = 100,
            height = 50,
            border_radius = 8,
        },
    })
end

UI.render(Window({
    style = {
        bg_color = "#1f1f2e",
        width = 640,
        height = 420,
        padding = 10,
        border_radius = 8,
        flex_direction = "column",
        gap = 8,
    },
    children = {
        View({
            style = { bg_color = "#2f2f45", width = 600, height = 120, border_radius = 12 },
            children = {
                ItemWidget({ color = function() return Store.items[1].color end }),
            },
        }),
        View({
            style = { bg_color = "#3a7ca5", width = 600, height = 90, border_radius = 12 },
        }),
    },
}))
