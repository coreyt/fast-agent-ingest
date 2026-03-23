// Auto-generated via interoptopus + manual polish.
// Re-generate with: cbindgen --config cbindgen.toml (Rust side)
// then run the interoptopus generator.
using System.Runtime.InteropServices;

namespace FastAgentIngest;

/// <summary>
/// Raw P/Invoke declarations for the fast-agent-ingest C ABI.
/// Do not call these directly — use <see cref="Converter"/> instead.
/// </summary>
internal static unsafe class NativeMethods
{
    // The native library name without prefix/extension — the runtime resolves
    // libfast_agent_ingest_c_ffi.so / fast_agent_ingest_c_ffi.dll automatically.
    private const string LibName = "fast_agent_ingest_c_ffi";

    [StructLayout(LayoutKind.Sequential)]
    internal struct FaiOptions
    {
        public byte ExtractMainContent;
        public byte IncludeImages;
        public byte IncludeLinks;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct FaiResult
    {
        public nint Markdown;    // *mut c_char — free with fai_free_string
        public nint Title;       // *mut c_char — may be null
        public nint Description; // *mut c_char — may be null
    }

    [DllImport(LibName, EntryPoint = "fai_convert", ExactSpelling = true)]
    internal static extern FaiResult FaiConvert(
        [MarshalAs(UnmanagedType.LPUTF8Str)] string html,
        in FaiOptions options);

    [DllImport(LibName, EntryPoint = "fai_free_string", ExactSpelling = true)]
    internal static extern void FaiFreeString(nint ptr);

    [DllImport(LibName, EntryPoint = "fai_free_result", ExactSpelling = true)]
    internal static extern void FaiFreeResult(FaiResult result);
}
