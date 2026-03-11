local function ClockWidget()
    return Row {
        id = "clock_widget",
        style = {
            flex_direction = "row",
            gap = 6,
            align_items = "center",
        },
        children = {
            Text {
                value = os.date("%H:%M"),
                style = { color = "#cdd6f4", font_size = 14 },
            },
        },
    }
end

UI.render(Window {
    id = "status_bar_root",
    anchor = "top",
    style = {
        flex_direction = "row",
        gap = 12,
        padding = "8px 16px",
        bg_color = "rgba(16, 19, 27, 0.8)",
        border_radius = 10,
    },
    children = {
        ClockWidget(),
        View { style = { flex_grow = 1 } },
        Text {
            value = "XFw",
            style = { color = "#89b4fa", font_size = 14 },
        },
    },
})
