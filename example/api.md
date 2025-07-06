get_call_data
```bash
curl -X POST "http://localhost:1234/get_calldata" \
  -H "Content-Type: application/json" \
  -d '{
    "chain_id": 1,
    "address": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    "function_name": "paprika_guessed_70a08231",
    "params": [
      {
        "content": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "params_type": "address"
      }
    ]
  }'
```

get_ui_abi
```bash
curl -X POST "http://localhost:1234/get_ui_abi" \
  -H "Content-Type: application/json" \
  -d '{
    "chain_id": 1,
    "address": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
  }'
```

compare_by_cfg
```bash
curl -X POST "http://localhost:1234/compare_by_cfg" \
  -H "Content-Type: application/json" \
  -d '{
    "chain_id": 1,
    "address1": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    "address2": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
  }'
```

get_json_abi
```bash
curl -X POST "http://localhost:1234/get_json_abi" \
  -H "Content-Type: application/json" \
  -d '{
    "chain_id": 1,
    "address": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
  }'
```

disassemble
```bash
curl -X POST "http://localhost:1234/disassemble" \
  -H "Content-Type: application/json" \
  -d '{
    "chain_id": 1,
    "address1": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    "address2": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
  }'
```

get_signature_by_selector
```bash
curl -X POST "http://localhost:1234/get_signature_by_selector" \
  -H "Content-Type: application/json" \
  -d '{
    "chain_id": 1,
    "address": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
  }'
```

get_guess_magic_result
```bash
curl -X POST "http://localhost:1234/get_guess_magic_result" \
  -H "Content-Type: application/json" \
  -d '{
    "hex_string": "000000000000000000000000000000000000000100061fa96e4dde0b9861300e00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000017000000000000000000000000000000000000000000000000000000000000012c000000000000000000000000000000000000000000000000000000000000012c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"
  }'
```

simulate_call
```bash
curl -X POST "http://localhost:1234/simulate_call" \
  -H "Content-Type: application/json" \
  -d '{
    "chain_id": 1,
    "from": "0xd3E65149C212902749D49011B6ab24bba30D97c6",
    "to": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    "function_name": "paprika_guessed_70a08231",
    "params": [
      {
        "content": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "params_type": "address"
      }
    ]
  }'
```