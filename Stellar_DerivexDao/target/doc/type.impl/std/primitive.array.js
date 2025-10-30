(function() {
    var type_impls = Object.fromEntries([["ed25519",[]],["ed25519_dalek",[]],["ring",[]],["uuid",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[14,21,12,12]}