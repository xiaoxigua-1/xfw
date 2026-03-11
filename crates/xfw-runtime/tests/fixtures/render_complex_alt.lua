UI.render(Window({
    style = {
        bg_color = "#17171f",
        width = 1080,
        height = 1920,
        padding = 10,
        border_radius = 12,
        flex_direction = "column",
        gap = 10,
    },
    children = {
        Row({
            style = { bg_color = "#1f2633", width = 1000, height = 160, border_radius = 16, gap = 16, overflow = "hidden" },
            children = {
                Column({
                    style = { gap = 8 },
                    children = {
                        Text({ value = "Section: Overview", style = { color = "#ffffff", font_size = 28 } }),
                        Text({ value = "Status: OK", style = { color = "#90be6d", font_size = 24 } }),
                    },
                }),
                View({ style = { bg_color = "#f9c74f", width = 140, height = 100, border_radius = 16 } }),
                View({ style = { bg_color = "#f8961e", width = 260, height = 140, border_radius = 16 } }),
            },
        }),
        Row({
            style = { bg_color = "#26263a", width = 1000, height = 220, border_radius = 16, gap = 20, clip = true },
            children = {
                Column({
                    style = { gap = 10 },
                    children = {
                        Text({ value = "Header", style = { color = "#ffffff", font_size = 32 } }),
                        Text({ value = "Subheading", style = { color = "#c7c7d9", font_size = 26 } }),
                    },
                }),
                View({ style = { bg_color = "#f9c74f", width = 220, height = 140, border_radius = 16 } }),
                View({ style = { bg_color = "#90be6d", width = 160, height = 140, border_radius = 16 } }),
                View({ style = { bg_color = "#43aa8b", width = 240, height = 180, border_radius = 16 } }),
            },
        }),
        Column({
            style = { bg_color = "#2f2f45", width = 1000, height = 320, border_radius = 16, gap = 14, overflow = "hidden" },
            children = {
                Row({
                    style = { gap = 16 },
                    children = {
                        Text({ value = "Left", style = { color = "#ffffff", font_size = 26 } }),
                        Text({ value = "Center", style = { color = "#9ad1d4", font_size = 26 } }),
                        Text({ value = "Right", style = { color = "#ffffff", font_size = 26 } }),
                    },
                }),
                Row({
                    style = { gap = 16 },
                    children = {
                        Text({ value = "Wrap test line one", style = { color = "#f1f1f5", font_size = 22 } }),
                        Text({ value = "Wrap test line two", style = { color = "#f1f1f5", font_size = 22 } }),
                    },
                }),
                Row({
                    style = { gap = 16 },
                    children = {
                        View({ style = { bg_color = "#577590", width = 360, height = 180, border_radius = 16 } }),
                        View({ style = { bg_color = "#f3722c", width = 360, height = 180, border_radius = 16 } }),
                        View({ style = { bg_color = "#f9844a", width = 320, height = 180, border_radius = 16 } }),
                    },
                }),
            },
        }),
        Row({
            style = { bg_color = "#3a506b", width = 1000, height = 220, border_radius = 16, gap = 16, overflow = "hidden" },
            children = {
                Column({
                    style = { gap = 8 },
                    children = {
                        Text({ value = "Footer", style = { color = "#ffffff", font_size = 26 } }),
                        Text({ value = "Meta", style = { color = "#d6d6e0", font_size = 22 } }),
                        Text({ value = "Note", style = { color = "#ffd166", font_size = 22 } }),
                    },
                }),
                View({ style = { bg_color = "#5bc0be", width = 240, height = 140, border_radius = 16 } }),
                View({ style = { bg_color = "#f94144", width = 140, height = 140, border_radius = 16 } }),
                View({ style = { bg_color = "#9b5de5", width = 200, height = 140, border_radius = 16 } }),
                View({ style = { bg_color = "#277da1", width = 240, height = 180, border_radius = 16 } }),
            },
        }),
    },
}))
