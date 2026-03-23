using System.Runtime.InteropServices;

namespace FastAgentIngest;

/// <summary>Options controlling HTML → Markdown conversion.</summary>
public sealed class ConversionOptions
{
    /// <summary>Run readability-style main-content extraction. Default: true.</summary>
    public bool ExtractMainContent { get; init; } = true;
    /// <summary>Emit Markdown image syntax. Default: true.</summary>
    public bool IncludeImages { get; init; } = true;
    /// <summary>Emit Markdown link syntax. Default: true.</summary>
    public bool IncludeLinks { get; init; } = true;

    public static readonly ConversionOptions Default = new();
}

/// <summary>Result of a conversion operation.</summary>
public sealed class ConversionResult
{
    public string Markdown { get; init; } = string.Empty;
    public string? Title { get; init; }
    public string? Description { get; init; }
}

/// <summary>
/// Converts HTML to Markdown using the fast-agent-ingest native library.
/// </summary>
public static class Converter
{
    /// <summary>Convert an HTML string to Markdown.</summary>
    /// <param name="html">The HTML to convert.</param>
    /// <param name="options">Conversion options. Null uses defaults.</param>
    public static ConversionResult Convert(string html, ConversionOptions? options = null)
    {
        options ??= ConversionOptions.Default;

        var nativeOpts = new NativeMethods.FaiOptions
        {
            ExtractMainContent = (byte)(options.ExtractMainContent ? 1 : 0),
            IncludeImages      = (byte)(options.IncludeImages ? 1 : 0),
            IncludeLinks       = (byte)(options.IncludeLinks ? 1 : 0),
        };

        var raw = NativeMethods.FaiConvert(html, in nativeOpts);
        try
        {
            return new ConversionResult
            {
                Markdown    = MarshalUtf8(raw.Markdown) ?? string.Empty,
                Title       = MarshalUtf8(raw.Title),
                Description = MarshalUtf8(raw.Description),
            };
        }
        finally
        {
            NativeMethods.FaiFreeResult(raw);
        }
    }

    private static string? MarshalUtf8(nint ptr)
    {
        if (ptr == nint.Zero) return null;
        return Marshal.PtrToStringUTF8(ptr);
    }
}
