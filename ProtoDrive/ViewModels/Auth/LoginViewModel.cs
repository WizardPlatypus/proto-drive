using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Core.Interfaces;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ViewModels.Base;
using CommunityToolkit.Mvvm.Input;
using ViewModels.FileExplorer;

namespace ViewModels.Auth
{
    public partial class LoginViewModel : ViewModelBase
    {
        [ObservableProperty]
        private string username;

        [ObservableProperty]
        private string password;

        public LoginViewModel(IApiService apiService, IDialogService dialogService, INavigationService navigationService)
            : base(apiService, dialogService, navigationService) { }

        [RelayCommand]
        private async Task Login()
        {
            if (string.IsNullOrWhiteSpace(Username) || string.IsNullOrWhiteSpace(Password))
            {
                _dialogService.ShowError("Username and password are required.");
                return;
            }
            try
            {
                string _token = await _apiService.LoginAsync(Username, Password);
                _navigationService.NavigateTo<FileExplorerViewModel>();
            }
            catch (UnauthorizedAccessException)
            {
                _dialogService.ShowError("Invalid credentials provided.");
            }
            catch (Exception ex)
            {
                _dialogService.ShowError($"An unexpected error occurred: {ex.Message}");
            }
        }
    }
}
