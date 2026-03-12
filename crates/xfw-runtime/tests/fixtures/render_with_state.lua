local Store = UI.state({
  items = {
    { id = "item1", color = "#ff9a3c" },
    { id = "item2", color = "#3a7ca5" },
  },
})

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
      id = 1,
      style = { bg_color = "#2f2f45", width = 600, height = 120, border_radius = 12 },
      children = {
        View({
          id = 2,
          style = {
            bg_color = function() return Store.items[1].color end,
            width = 100,
            height = 50,
            border_radius = 8,
          },
        }),
      },
    }),
    View({
      style = { bg_color = "#3a7ca5", width = 600, height = 90, border_radius = 12 },
    }),
  },
}))
