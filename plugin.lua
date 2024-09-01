assert(plugin, "This is meant to be installed as a plugin!")

local HttpService = game:GetService("HttpService")
local RunService = game:GetService("RunService")

if not RunService:IsEdit() or RunService:IsRunning() then return end

local Toolbar = plugin:CreateToolbar("Wake")
local EnableButton = Toolbar:CreateButton("Enable", "Enable the plugin", "rbxassetid://4458901694")
EnableButton.ClickableWhenViewportHidden = true

local ATTRIBUTE = "IS_PROJECT"
local PLUGIN_ENABLED = workspace:GetAttribute(ATTRIBUTE) or false
local IS_IN_EDIT = true
local LAST_POLL = tick()

local DEFAULT_ADDRESS = "127.0.0.1"
local DEFAULT_PORT = "9595"
local DEFAULT_URL = string.format("http://%s:%s", DEFAULT_ADDRESS, DEFAULT_PORT)

EnableButton:SetActive(PLUGIN_ENABLED)

EnableButton.Click:Connect(function()
    if RunService:IsEdit() and not RunService:IsRunning() then
        EnableButton:SetActive(not PLUGIN_ENABLED)
        PLUGIN_ENABLED = not PLUGIN_ENABLED

        if PLUGIN_ENABLED then
            workspace:SetAttribute(ATTRIBUTE, true)
            Sync()
        else
            workspace:SetAttribute(ATTRIBUTE, nil)
            Close()
        end
    end
end)

local function Request(Method, URL)
    local Success, Result = pcall(function()
        return HttpService:RequestAsync({
            Url = URL,
            Method = Method
        })
    end)

    if Success then
        return Result
    end

    return Success
end

local function time_since_last_poll()
    return tick() - LAST_POLL
end

function Poll()
    LAST_POLL = tick()
    local Result = Request("PUT", DEFAULT_URL)

    if Result then
        print(Result)
    end
end

function Close()
    local Result = Request("DELETE", DEFAULT_URL)

    if Result and time_since_last_poll() > 2 then
        Poll()
    end
end

function Sync()
    local Result = Request("PATCH", DEFAULT_URL)

    if Result then
        Poll()
    end
end

RunService.Heartbeat:Connect(function()
    if PLUGIN_ENABLED and not RunService:IsRunning() then
        IS_IN_EDIT = true
        if time_since_last_poll() > 2 then
            Poll()
        end
    elseif PLUGIN_ENABLED and RunService:IsRunning() and IS_IN_EDIT then
        IS_IN_EDIT = false
        Close()
    end
end)

plugin.Unloading:Connect(function()
    if PLUGIN_ENABLED then Close() end
end)

if PLUGIN_ENABLED then
    Sync()
end