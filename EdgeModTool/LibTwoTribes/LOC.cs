using System.Text;
using System.Text.Encodings.Web;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace EdgeModTool.LibTwoTribes;

internal sealed class LOC {
    private static readonly JsonSerializerOptions Jso = new() {
        Encoder = JavaScriptEncoder.UnsafeRelaxedJsonEscaping,
        WriteIndented = true
    };

    private LOC(Stream stream) {
        using BinaryReader br = new(stream, Encoding.Unicode, true);
        var numlangs = br.ReadInt16();
        List<string> languages = new(numlangs);

        for (var i = 0; i < numlangs; i++) {
            languages.Add(Encoding.ASCII.GetString(br.ReadBytes(2)));
        }

        var numstrings = br.ReadInt16();
        NewStringData = new(numstrings);

        for (var i = 0; i < numstrings; i++) {
            var key = br.ReadUInt32();
            Dictionary<string, string> value = new(numlangs);

            for (var j = 0; j < numlangs; j++) {
                var stringLength = br.ReadInt16();
                var stringValue = Encoding.Unicode.GetString(br.ReadBytes(stringLength * 2));

                value.Add(languages[j], stringValue.TrimEnd('\0'));
            }

            NewStringData.Add(key, value);
        }
    }

    private LOC(Dictionary<uint, Dictionary<string, string>> stringData) => NewStringData = stringData;

    public Dictionary<uint, Dictionary<string, string>> NewStringData { get; }

    public static LOC FromFile(string path) {
        using FileStream fs = new(path, FileMode.Open, FileAccess.Read);

        return FromStream(fs);
    }

    public static LOC FromStream(Stream stream) => new(stream);

    public static LOC FromJson(ReadOnlySpan<byte> utf8Json) {
        var rootJo = JsonNode.Parse(utf8Json)!.AsObject();
        Dictionary<uint, Dictionary<string, string>> strings = [];

        foreach (var kv1 in rootJo) {
            var stringKey = uint.Parse(kv1.Key);
            var stringObject = kv1.Value!.AsObject();
            Dictionary<string, string> langValue = [];

            foreach (var (language, value) in stringObject) {
                var stringValue = value!.AsValue().GetValue<string>();

                langValue.Add(language, stringValue);
            }

            strings.Add(stringKey, langValue);
        }

        return new(strings);
    }

    public void Save(string path) {
        using FileStream fsOut = new(path, FileMode.Create, FileAccess.Write);

        Save(fsOut);
    }

    public void Save(Stream stream) {
        using BinaryWriter bw = new(stream, Encoding.Unicode, true);
        var languages = NewStringData.First().Value.Keys.ToArray();

        bw.Write((short)languages.Length);

        foreach (var lang in languages) {
            bw.Write(Encoding.ASCII.GetBytes(lang));
        }

        bw.Write((short)NewStringData.Count);

        var stringKeys = NewStringData.Keys.ToArray();

        for (var i = 0; i < NewStringData.Count; i++) {
            bw.Write(stringKeys[i]);

            foreach (var language in languages) {
                var value = NewStringData[stringKeys[i]][language] + '\0';

                bw.Write((short)value.Length);

                foreach (var t in value) {
                    bw.Write(t);
                }
            }
        }
    }

    public void SaveJson(string path) => File.WriteAllText(path, JsonSerializer.Serialize(NewStringData, Jso));
}
