### Solana Test Validator

```sh
solana-test-validator
```

Fetch some account:

```sh
solana account SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf
```

### Surfpool Mainnet Forking

Start surfpool in non-anchor dir

```sh
surfpool start
```

```sh
solana account SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf
```

### Surfpool with anchor project

Go to project terminal:

```sh
surfpool start --watch
```

- Create runbook
- show deployment
- recompile to show auto-redeploy

### Update devnet signer

```hcl
signer "payer" "svm::web_wallet" {
	expected_address = "zbBjhHwuqyKMmz8ber5oUtJJ3ZV4B6ePmANfGyKzVGV"
}

signer "authority" "svm::squads" {
  address = "4xAJhZCxoZKKXNN5Rh3uTzV4r8QkHp4ToHCvw4whrnJZ"
}
```

Runbook with subgraph:

```hcl

################################################################
# Manage solana-foundation-demo deployment through Crypto Infrastructure as Code
################################################################

addon "svm" {
    rpc_api_url = input.rpc_api_url
}

variable "program" {
    value = svm::get_program_from_anchor_project("goodbye_cruel_world")
}

action "deploy_goodbye_cruel_world" "svm::deploy_program" {
    description = "Deploy goodbye_cruel_world program"
    program = variable.program
    authority = signer.deployer
    payer = signer.deployer
}

action "transfer_event" "svm::deploy_subgraph" {
    description = "Deploy a subgraph of TransferEvent calls"
    block_height = 0
    program_id = action.deploy_goodbye_cruel_world.program_id
    source = svm::get_idl_event(variable.program.idl, "TransferEvent")
    field {
        name = "amount"
    }
    field {
        name = "recipient"
        source = "to_pubkey"
    }
}

```

Runbook to call contract:

```hcl

addon "svm" {
    rpc_api_url = input.rpc_api_url
}

signer "caller" "svm::secret_key" {
    keypair_json = input.authority_keypair_json
}

variable "program" {
    description = "The program to deploy"
    value = svm::get_program_from_anchor_project("goodbye_cruel_world")
}

variable "data" {
    description = "The data for the 'hello' instruction"
    value = svm::get_instruction_data_from_idl(
		variable.program.idl,
		"hello",
		["ludo", 100]
	)
}

variable "event_authority" {
    description = "The PDA of the event authority"
    value = svm::find_pda(variable.program.program_id, [std::encode_hex("__event_authority")])
}

action "call" "svm::process_instructions" {
	description = "Invoke the 'hello' instruction"
	signers = [signer.caller]

	instruction {
		program_id = variable.program.program_id
		account {
			public_key = signer.caller.public_key
			is_signer = true
			is_writable = true
		}
		account {
			public_key = "FbFycRH7YUPNvVUdRMVdyUCVJEzmaUVoEepFRLUb6r3n"
			is_writable = true
		}
        account {
            public_key = svm::system_program_id()
        }
        account {
            public_key = variable.event_authority.pda
        }
        account {
            public_key = variable.program.program_id
        }
		data = variable.data
	}
}

output "signature" {
    description = "The signature of the transaction"
    value = action.call.signature
}

```

```yaml
- name: call
  description: Call the program
  location: runbooks/call.tx
```

```hcl

action "message_event" "svm::deploy_subgraph" {
    description = "Deploy a subgraph of MessageEvent calls"
    block_height = 0
    program_id = action.deploy_goodbye_cruel_world.program_id
    source = svm::get_idl_event(variable.program.idl, "MessageEvent")
    field {
        name = "message"
    }
}

```
