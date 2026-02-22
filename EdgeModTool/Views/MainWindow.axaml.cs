using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Interactivity;
using Avalonia.Platform.Storage;
using EdgeModTool.LibTwoTribes;
using System.Diagnostics.CodeAnalysis;

namespace EdgeModTool.Views;

internal sealed partial class MainWindow : Window {
    public MainWindow() {
        InitializeComponent();
        AddHandler(DragDrop.DragOverEvent, DragOverHandler);
        AddHandler(DragDrop.DropEvent, DropHandler);
    }

    private static bool IsSingleFile([NotNullWhen(true)] IStorageItem[]? items) => items?.Length == 1 && File.Exists(items[0].Path.LocalPath);

    private void DragOverHandler(object? sender, DragEventArgs e) {
        var files = e.DataTransfer.TryGetFiles();

        e.DragEffects = IsSingleFile(files) ? DragDropEffects.Copy : DragDropEffects.None;
    }

    private void DropHandler(object? sender, DragEventArgs e) {
        var files = e.DataTransfer.TryGetFiles();

        if (!IsSingleFile(files)) {
            return;
        }

        var filePath = files[0].Path.LocalPath;

        if (Path.GetFileName(filePath).Equals("text.loc", StringComparison.OrdinalIgnoreCase)) {
            var loc = LOC.FromFile(filePath);
        }
    }

    private async void Button_OnClick(object? sender, RoutedEventArgs e) {
        var files = await StorageProvider.OpenFilePickerAsync(new() {
            AllowMultiple = false
        });

        var localPath = files[0].Path.LocalPath;

        switch (Path.GetFileName(files[0].Path.LocalPath)) {
            case "text.loc":
                var loc = LOC.FromFile(localPath);

                loc.SaveJson("/mnt/data/temp/test.json");

                break;

            case "text.json":
                var newLoc = LOC.FromJson(await File.ReadAllBytesAsync(localPath));

                newLoc.Save("/mnt/data/temp/text.loc");

                break;

            case "font.bin":
            case "font.xml":
                Compiler.Compile(localPath);

                break;
        }
    }
}
