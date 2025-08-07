module 0x1::create_signer {
        friend 0x1::account;
        friend 0x1::aptos_account;
        friend 0x1::genesis;
        friend 0x1::multisig_account;
        friend 0x1::object;
    native public(friend) fun create_signer(arg0: address) : signer;
    // decompiled from Move bytecode v6
}
