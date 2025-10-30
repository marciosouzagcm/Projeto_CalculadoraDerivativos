(function() {
    var type_impls = Object.fromEntries([["clipboard_win",[]],["error_code",[]],["libc",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[20,18,12]}