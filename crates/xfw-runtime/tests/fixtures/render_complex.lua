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
                View({ style = { bg_color = "#ff9a3c", width = 140, height = 84, border_radius = 10 } }),
            },
        }),
        View({
            style = { bg_color = "#3a7ca5", width = 600, height = 90, border_radius = 12 },
        }),
        View({
            style = { bg_color = "#6a4c93", width = 600, height = 90, border_radius = 12 },
            children = {
                View({ style = { bg_color = "#43aa8b", width = 180, height = 54, border_radius = 8 } }),
                View({ style = { bg_color = "#f94144", width = 90, height = 54, border_radius = 8 } }),
            },
        }),
    },
}))
