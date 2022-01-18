require('tmcrud')

box.cfg({listen = 3301})
box.schema.func.create('tmcrud', {language = 'C', if_not_exists = true})
box.schema.func.create('tmcrud.read', {language = 'C', if_not_exists = true})
box.schema.func.create('tmcrud.read_at', {language = 'C', if_not_exists = true})
box.schema.func.create('tmcrud.insert', {language = 'C', if_not_exists = true})
box.schema.user.grant('guest', 'execute', 'function', 'tmcrud', {if_not_exists = true})
box.schema.user.grant('guest', 'execute', 'function', 'tmcrud.read', {if_not_exists = true})
box.schema.user.grant('guest', 'execute', 'function', 'tmcrud.read_at', {if_not_exists = true})
box.schema.user.grant('guest', 'execute', 'function', 'tmcrud.insert', {if_not_exists = true})

local customers_space = box.schema.space.create('forecast_city', {
    format = {
        {name = 'city_id', type = 'unsigned'},
        {name = 'ts', type = 'unsigned'},
        --{name = 'bucket_id', type = 'unsigned'},
        {name = 'temp', type = 'double'},
        {name = 'pressure', type = 'double'},
    },
    if_not_exists = true,
})
customers_space:create_index('id', {
    parts = { {field ='city_id', is_nullable = false}, {field ='ts', is_nullable = false} },
    if_not_exists = true,
})
--customers_space:create_index('bucket_id', {
--    parts = { {field ='bucket_id', is_nullable = false} },
--    if_not_exists = true,
--})

box.once('fixture', function()
    box.space.forecast_city:insert{5000,1642496549, 10.1, 0.4}
    box.space.forecast_city:insert{5001,1642496549, 14.9, 0.1}
    box.space.forecast_city:insert{5000,1642582949, 13.2, 0.4}
    box.space.forecast_city:insert{5001,1642582949, 14.1, 0.2}
end)


local console = require('console')
console.start()
