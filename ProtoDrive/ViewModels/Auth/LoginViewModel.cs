using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Core.Interfaces;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ViewModels.Base;
using ViewModels.FileExplorer;

namespace ViewModels.Auth
{
    public partial class LoginViewModel : ViewModelBase, IInitializable
    {
        [ObservableProperty]
        private string username = "";

        [ObservableProperty]
        private string password = "";

        public LoginViewModel(IApiService apiService, IDialogService dialogService, INavigationService navigatorService) : base(apiService, dialogService, navigatorService) { }
        public LoginViewModel() : base(null!, null!, null!) { }

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
        [RelayCommand]
        private void NavigateToRegister()
        {
            _navigationService.NavigateTo<RegisterViewModel>();
        }

        public Task InitializeAsync(object? parameter = null)
        {
            return Task.Run(() =>
            {
                if (parameter != null && parameter is (String l, String p))
                {
                    Username = l;
                    Password = p;
                }
            });
        }
    }
}
