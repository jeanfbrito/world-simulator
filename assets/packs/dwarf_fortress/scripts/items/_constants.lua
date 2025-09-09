-- Shared constants for all item definitions

local M = {}

-- Resource categories for organization
M.categories = {
    RAW_MATERIAL = "raw_material",
    PROCESSED = "processed",
    FOOD = "food",
    RARE = "rare",
    TOOL = "tool",
    TRADE = "trade_good"
}

-- Quality tiers that affect value and effectiveness
M.quality = {
    POOR = 0.7,
    NORMAL = 1.0,
    GOOD = 1.3,
    EXCELLENT = 1.6,
    MASTERWORK = 2.0
}

-- Seasons for harvest modifiers
M.seasons = {
    SPRING = "spring",
    SUMMER = "summer",
    AUTUMN = "autumn",
    WINTER = "winter"
}

return M