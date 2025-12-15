using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Core.Interfaces
{
    public interface IDialogService
    {
        Task<bool> ShowConfirmationAsync(string title, string message);
        void ShowError(string message);
    }
}
