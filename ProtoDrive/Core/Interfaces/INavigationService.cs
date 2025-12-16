using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ProtoDrive.Core.Interfaces
{
    public interface INavigationService
    {
        /// <summary>
        /// Navigates the application's primary frame/window to the specified view model type.
        /// </summary>
        /// <typeparam name="TViewModel">The type of the ViewModel to navigate to (e.g., FileExplorerViewModel).</typeparam>
        void NavigateTo<TViewModel>() where TViewModel : class, IViewModel;

        /// <summary>
        /// Navigates to a view model type and passes an object as initialization data.
        /// </summary>
        void NavigateTo<TViewModel>(object? parameter) where TViewModel : class, IViewModel;
        Type GetViewTypeForViewModel(Type viewModelType);

        /// <summary>
        /// Event that the View (e.g., MainWindow) subscribes to, triggering the actual UI change.
        /// </summary>
        event Action<Type, object>? NavigationRequested;
    }
}
