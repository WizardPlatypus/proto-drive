using CommunityToolkit.Mvvm.ComponentModel;
using Core.Interfaces;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ViewModels.Base
{
    public partial class ViewModelBase : ObservableObject, IViewModel
    {
        protected readonly IApiService _apiService;
        protected readonly IDialogService _dialogService;
        protected readonly INavigationService _navigationService;

        public ViewModelBase(IApiService apiService, IDialogService dialogService, INavigationService navigatorService)
        {
            _apiService = apiService;
            _dialogService = dialogService;
            _navigationService = navigatorService;
        }
    }
}
