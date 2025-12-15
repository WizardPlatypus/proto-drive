using Core.Interfaces;
using Desktop.Services;
using Microsoft.Extensions.DependencyInjection;
using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using ViewModels.Auth;

namespace Desktop
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        private readonly IServiceProvider _serviceProvider;
        private readonly WpfNavigationService _navigationService;
        public MainWindow(INavigationService navigationService, IServiceProvider serviceProvider)
        {
            InitializeComponent();
            _serviceProvider = serviceProvider;
            _navigationService = (WpfNavigationService)navigationService;
            _navigationService.NavigationRequested += OnNavigationRequested;
            _navigationService.NavigateTo<LoginViewModel>();
        }
        private void OnNavigationRequested(Type viewModelType, object? parameter)
        {
            Type viewType = _navigationService.GetViewTypeForViewModel(viewModelType);
            var viewModelInstance = _serviceProvider.GetRequiredService(viewModelType);
            var viewInstance = (FrameworkElement)_serviceProvider.GetRequiredService(viewType);
            // 4. Handle parameter (if you use IInitializable on your ViewModel)
            //if (parameter != null && viewModelInstance is IInitializable initVm)
            //{
            //    initVm.Initialize(parameter);
            //}
            viewInstance.DataContext = viewModelInstance;
            NavigationFrame.Content = viewInstance;
        }
    }
}