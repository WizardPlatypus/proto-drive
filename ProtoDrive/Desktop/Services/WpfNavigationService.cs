using Core.Interfaces;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ViewModels.Auth;
using ViewModels.FileExplorer;

namespace Desktop.Services
{
    class WpfNavigationService : INavigationService
    {
        private readonly Dictionary<Type, Type> _mapping = new();
        private readonly IServiceProvider _serviceProvider;
        public event Action<Type, object> NavigationRequested;

        public WpfNavigationService(IServiceProvider serviceProvider)
        {
            _serviceProvider = serviceProvider;
            Map<LoginViewModel, Views.LoginPage>();
            Map<RegisterViewModel, Views.RegisterPage>();
            Map<FileExplorerViewModel, Views.FileExplorerPage>();
        }
        void Map<TViewModel, TView>()
            where TViewModel : IViewModel
            where TView : IView, new()
        {
            _mapping[typeof(TViewModel)] = typeof(TView);
        }

        public void NavigateTo<TViewModel>() where TViewModel : class, IViewModel
        {
            NavigateTo<TViewModel>(null);
        }

        public void NavigateTo<TViewModel>(object? parameter) where TViewModel : class, IViewModel
        {
            NavigationRequested?.Invoke(typeof(TViewModel), parameter);
            throw new NotImplementedException();
        }

        public Type GetViewTypeForViewModel(Type viewModelType)
        {
            if (_mapping.TryGetValue(viewModelType, out var viewType))
            {
                return viewType;
            }
            throw new KeyNotFoundException($"No view registered for ViewModel type: {viewModelType.Name}.");
        }
    }
}
