using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using ProtoDrive.Core.Interfaces;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ProtoDrive.ViewModels.Base;

namespace ProtoDrive.ViewModels.FileExplorer
{
    public partial class FileExplorerViewModel : ViewModelBase, IInitializable
    {
        public ObservableCollection<FileViewModel> CurrentFolderContents { get; } = new();
        private int _currentPage = 0;
        private const int PageSize = 50;
        [ObservableProperty]
        private string path = "";
        [ObservableProperty]
        private bool isBusy = false;

        public FileExplorerViewModel(IApiService apiService, IDialogService dialogService, INavigationService navigatorService) : base(apiService, dialogService, navigatorService) { }
        public FileExplorerViewModel() : base(null!, null!, null!) { }

        [RelayCommand]
        private async Task LoadFolderContents(Guid folderId)
        {
            try
            {
                this.IsBusy = true;
                _currentPage = 0;
                CurrentFolderContents.Clear();
                var items = await _apiService.GetFolderContentsAsync(folderId);
                foreach (var item in items)
                {
                    CurrentFolderContents.Add(new FileViewModel(item));
                }
            }
            catch (Exception ex)
            {
                _dialogService.ShowError($"Failed to load contents: {ex.Message}");
            }
            finally { this.IsBusy = false; }
        }
        [RelayCommand]
        private async Task OpenFolder()
        {
            try
            {
                IsBusy = true;
                _currentPage = 0;
                CurrentFolderContents.Clear();
                var items = await _apiService.GetFolderContentsAsync(Path);
                foreach (var item in items)
                {
                    CurrentFolderContents.Add(new FileViewModel(item));
                }
            }
            catch (Exception ex)
            {
                _dialogService.ShowError($"Failed to load contents: {ex.Message}");
            }
            finally { IsBusy = false; }
        }

        public async Task InitializeAsync(object? parameter = null)
        {
            Guid root = parameter is Guid ? (Guid)parameter : Guid.Empty;
            await LoadFolderContents(root);
        }
    }
}
