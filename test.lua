wrk.method = "POST"
wrk.body   = '{"jsonrpc":"2.0","method":"get_block_height","id":1}'
wrk.headers["Content-Type"] = "application/json"
