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
        public RegisterViewModel(IApiService apiService, IDialogService dialogService, INavigationService navigatorService) : base(apiService, dialogService, navigatorService) { }
        [RelayCommand]
        private void NavigateToLogin()
        {
            _navigationService.NavigateTo<LoginViewModel>();
        }
        // TODO: implement the Register handler
        // Copy most code from LoginViewModel
    }
}
