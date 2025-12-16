using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Core.Interfaces;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ViewModels.Base;

namespace ViewModels.Auth
{
    public partial class RegisterViewModel : ViewModelBase
    {
        [ObservableProperty]
        private string username = "";

        [ObservableProperty]
        private string password = "";

        [ObservableProperty]
        private string repeat = "";

        public RegisterViewModel(IApiService apiService, IDialogService dialogService, INavigationService navigatorService) : base(apiService, dialogService, navigatorService) { }
        public RegisterViewModel() : base(null!, null!, null!) { }

        [RelayCommand]
        private async Task Register()
        {
            if (string.IsNullOrEmpty(Username) || string.IsNullOrEmpty(Password))
            {
                _dialogService.ShowError("Username and password are required");
                return;
            }
            if (Password != Repeat)
            {
                _dialogService.ShowError("Original password and the repeat differ");
                return;
            }
            try
            {
                await _apiService.RegisterAsync(Username, Password);
                _navigationService.NavigateTo<LoginViewModel>((Username, Password));
            }
            catch (InvalidOperationException ex)
            {
                _dialogService.ShowError($"Registration failed: {ex.Message}");
            }
            catch (Exception ex)
            {
                _dialogService.ShowError($"An unexpected error occurred: {ex.Message}");
            }
        }

        [RelayCommand]
        private void NavigateToLogin()
        {
            _navigationService.NavigateTo<LoginViewModel>();
        }
    }
}
