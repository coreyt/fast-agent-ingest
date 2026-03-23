using FastAgentIngest;
using Xunit;

namespace FastAgentIngest.Tests;

public class ConformanceTests
{
    private static readonly string FixturesRoot =
        Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", "..", "tests", "fixtures");

    [Fact]
    public void SimpleConvert()
    {
        var result = Converter.Convert("<html><body><h1>Hello</h1><p>World</p></body></html>");
        Assert.Contains("# Hello", result.Markdown);
        Assert.Contains("World", result.Markdown);
    }

    public static IEnumerable<object[]> FixtureNames()
    {
        var inputDir = Path.Combine(FixturesRoot, "inputs");
        if (!Directory.Exists(inputDir)) yield break;

        foreach (var file in Directory.GetFiles(inputDir, "*.html"))
        {
            yield return new object[] { Path.GetFileNameWithoutExtension(file) };
        }
    }

    [Theory]
    [MemberData(nameof(FixtureNames))]
    public void FixtureConformance(string name)
    {
        var html     = File.ReadAllText(Path.Combine(FixturesRoot, "inputs",   name + ".html"));
        var expected = File.ReadAllText(Path.Combine(FixturesRoot, "expected", name + ".md")).Trim();

        var result = Converter.Convert(html);
        Assert.Equal(expected, result.Markdown.Trim());
    }
}
