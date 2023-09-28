--[[function ColorMyPencils(color)
	color = color or "rose-pine"
	vim.cmd.colorscheme(color)

	vim.api.nvim_set_hl(0, "Normal", { bg = "none" })
	vim.api.nvim_set_hl(0, "NormalFloat", { bg = "none" })
end

ColorMyPencils()]]--
--[[require("gruvbox").setup({
    contrast = "",
    palette_overrides = {
        --bright_orange = "#d5c4a1",
        --neutral_orange = "#a89984",
        bright_yellow = "#f9f5d7"
    }
})
vim.o.background = "dark"
vim.cmd([[colorscheme gruvbox]])]]--
vim.cmd([[colorscheme solarized]])
