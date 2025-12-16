using ProtoDrive.Core.Interfaces;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;

namespace ProtoDrive.Desktop.Services
{
    class WpfDialogService : IDialogService
    {
        public Task<bool> ShowConfirmationAsync(string title, string message)
        {
            return Task.FromResult(
                MessageBox.Show(message, title, MessageBoxButton.YesNo, MessageBoxImage.Question) == MessageBoxResult.Yes
            );
        }

        public void ShowError(string message)
        {
            MessageBox.Show(message, "Error", MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }
}
