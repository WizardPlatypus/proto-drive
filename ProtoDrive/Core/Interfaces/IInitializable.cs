using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Core.Interfaces
{
    public interface IInitializable
    {
        /// <summary>
        /// Initializes the ViewModel asynchronously with optional navigation parameters.
        /// </summary>
        /// <param name="parameter">An optional object passed during navigation.</param>
        Task InitializeAsync(object? parameter = null);
    }
}
