assert plugin, "This must be installed as a plugin!"

http_service = game\GetService "HttpService"
run_service = game\GetService "RunService"
collection_service = game\GetService "CollectionService"

return if not run_service\isEdit! or run_service\isRunning!

toolbar = plugin\CreateToolbar "Wake"
enable_button = toolbar\CreateButton "Enable", "Enable the plugin", "rbxassetid://4458901694"
enable_button.ClickableWhenViewportHidden = true

get_hash = () -> math.random 0xff, 0xffffff

attribute = "IS_PROJECT"
enabled = workspace\GetAttribute(attribute) or false
is_in_edit = true
last_poll = tick()
hash = get_hash!

default_address = "127.0.0.1"
default_port = "9595"
default_url = "http://%s:%s"\format default_address, default_port

enable_button\SetActive enabled

-- see lua-users
decode = (data) ->
    b = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_'
    data = string.gsub data, "[^#{b}=]", ''
    (data\gsub '.', (x) ->
        return '' if x == '='
        r = ''
        f= (b\find x) - 1
        for i=6,1,-1
            r = "#{r}#{(f%2^i-f%2^(i-1)>0 and '1' or '0')}"
        r
    )\gsub '%d%d%d?%d?%d?%d?%d?%d?', (x) ->
        return '' if #x ~= 8
        c = 0
        for i = 1, 8
            c = c + ((x\sub i, i) == '1' and 2^(8-i) or 0)
        string.char c

request = (method, url) ->
    success, result = pcall () ->
        http_service\RequestAsync({
            Url: url,
            Method: method
        })

    return result if success
    success

create_script = (path, name, script_type, content) ->
    characters, index, tree = {}, 1, nil
    for i = 1, #path
        table.insert characters, path\sub i, i
    while characters[index]
        if characters[index] == '#'
            index += 1
            ancestor = ''
            while characters[index] and characters[index] ~= '#' and characters[index] ~= '!' and characters[index] ~= ':'  and characters[index] ~= '.'
                ancestor ..= characters[index]
                index += 1
            tree = game\GetService ancestor
        elseif characters[index] == '!'
            index += 1
            ancestor = ''
            while characters[index] and characters[index] ~= '#' and characters[index] ~= '!' and characters[index] ~= ':'  and characters[index] ~= '.'
                ancestor ..= characters[index]
                index += 1
            potential = tree\FindFirstChild ancestor
            tree = potential if potential
            tree = Instance.new "Folder", tree unless potential
            tree.Name = ancestor unless potential
            collection_service\AddTag tree, attribute unless potential
        elseif characters[index] == '.'
            index += 1
            ancestor = ''
            while characters[index] and characters[index] ~= '#' and characters[index] ~= '!' and characters[index] ~= ':'  and characters[index] ~= '.'
                ancestor ..= characters[index]
                index += 1
            potential = tree\FindFirstChild ancestor
            error "Could not find child '#{ancestor}, check if your configuration is correct'" unless potential
            tree = potential
        elseif characters[index] == ':'
            index += 1
            ancestor = ''
            while characters[index] and characters[index] ~= '#' and characters[index] ~= '!' and characters[index] ~= ':'  and characters[index] ~= '.'
                ancestor ..= characters[index]
                index += 1
            potential = tree\WaitForChild ancestor
            error "Could not find child '#{ancestor}, check if your configuration is correct'" unless potential
            tree = potential
        else
            error "Unknown character in path"
    unless (tree\FindFirstChild name) and (tree\FindFirstChild name)\IsA script_type
        if tree\FindFirstChild name
            (tree\FindFirstChild name)\Destroy!
        tree = Instance.new script_type, tree
        tree.Name = name
    else
        tree = tree\FindFirstChild name
    collection_service\AddTag tree, attribute
    tree.Source = content


last_success = false
handle = (response) ->
    last_success = false
    return unless response
    body = response.Body
    return unless body
    last_success = true

    characters, index = {}, 1
    for i = 1, #body
        table.insert characters, body\sub i, i

    if characters[index] == '^'
        for tag, tagged in collection_service\GetTagged attribute
            tagged\Destroy!
        handle request "PATCH", default_url
        last_poll = tick!
        return handle request "PUT", default_url
    while characters[index] == '{'
        index += 1
        path = ''
        while characters[index] ~= ':'
            path ..= characters[index]
            index += 1
        path = decode path
        index += 1

        name = ''
        while characters[index] ~= ','
            name ..= characters[index]
            index += 1
        name = decode name
        index += 1

        script_type = ''
        while characters[index] ~= ','
            script_type ..= characters[index]
            index += 1
        
        index += 1
        content = ''
        while characters[index] ~= '}'
            content ..= characters[index]
            index += 1
        content = decode content
        index += 1

        task.spawn () -> create_script path, name, script_type, content

since_last = () -> tick! - last_poll

poll = () ->
    last_poll = tick!
    unless last_success
        for tag, tagged in collection_service\GetTagged attribute
            tagged\Destroy!
        handle request "PATCH", default_url
    else
        handle request "PUT", default_url
close = () -> request "DELETE", default_url
sync = () ->
    for tag, tagged in collection_service\GetTagged attribute
        tagged\Destroy!
    handle request "PATCH", default_url
    poll!

run_service.Heartbeat\Connect () ->
    if enabled and not run_service\IsRunning()
        is_in_edit = true
        poll! if since_last! > 1
    elseif enabled and run_service\IsRunning! and is_in_edit
        is_in_edit = false
        close!

plugin.Unloading\Connect () -> close! if enabled

enable_button.Click\Connect () ->
    return if not run_service\isEdit! or run_service\isRunning!
    enable_button\SetActive not enabled
    enabled = not enabled

    if enabled
        workspace\SetAttribute attribute, true
        sync!
    else
        workspace\SetAttribute attribute
        close!