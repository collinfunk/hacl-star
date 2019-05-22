let _ =
  CmdLineParser.parse_cmdline [
      ("check_aesni",  (fun win -> Vale_Lib_X64_Cpuidstdcall.va_code_check_aesni_stdcall win), 0);
      ("check_sha",    (fun win -> Vale_Lib_X64_Cpuidstdcall.va_code_check_sha_stdcall win), 0);
      ("check_adx_bmi2",  (fun win -> Vale_Lib_X64_Cpuidstdcall.va_code_check_adx_bmi2_stdcall win), 0);
      ("check_avx",     (fun win -> Vale_Lib_X64_Cpuidstdcall.va_code_check_avx_stdcall win), 0);
      ("check_avx2",    (fun win -> Vale_Lib_X64_Cpuidstdcall.va_code_check_avx2_stdcall win), 0);
    ]
