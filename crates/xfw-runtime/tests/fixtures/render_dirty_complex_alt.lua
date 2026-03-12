local Store = UI.state({
    items = {
        { id = "item1", color = "#ff9a3c", visible = true },
        { id = "item2", color = "#3a7ca5", visible = true },
    },
    title = "Hello",
})

local function ItemWidget(props)
    if not props.visible then
        return nil
    end
    return View({
        style = {
            bg_color = "#50fa7b",
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
                ItemWidget({ color = Store.items[1].color, visible = Store.items[1].visible }),
            },
        }),
        View({
            style = { bg_color = "#3a7ca5", width = 600, height = 90, border_radius = 12 },
        }),
        View({
            style = { 
                bg_color = "#4a4a6a", 
                width = 600, 
                height = 60, 
                border_radius = 12,
                overflow = "hidden",
            },
            children = {
                ItemWidget({ color = Store.items[2].color, visible = Store.items[2].visible }),
            },
        }),
    },
}))
