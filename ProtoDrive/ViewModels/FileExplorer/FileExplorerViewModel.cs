using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Core.Interfaces;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ViewModels.Base;
using System.Collections.ObjectModel;

namespace ViewModels.FileExplorer
{
    public partial class FileExplorerViewModel : ViewModelBase
    {
        public ObservableCollection<Core.Domain.Entities.File> CurrentFolderContents { get; } = new();
        private int _currentPage = 0;
        private const int PageSize = 50;

        public FileExplorerViewModel(IApiService apiService, IDialogService dialogService, INavigationService navigatorService) : base(apiService, dialogService, navigatorService) { }

        [RelayCommand]
        private async Task LoadFolderContents(Guid folderId)
        {
            try
            {
                _currentPage = 0;
                CurrentFolderContents.Clear();
                var items = await _apiService.GetFolderContentsAsync(folderId);
                foreach (var item in items)
                {
                    CurrentFolderContents.Add(item);
                }
            }
            catch (Exception ex)
            {
                _dialogService.ShowError($"Failed to load contents: {ex.Message}");
            }
        }
    }
}
