using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading.Tasks;

namespace ProtoDrive.Core.Interfaces
{
    public interface IApiService
    {
        Task RegisterAsync(string login, string password);
        Task<string> LoginAsync(string login, string password);
        Task UploadFileAsync(Stream file, string name, string destination);
        Task<Stream> DownloadFileAsync(Guid fileId);
        Task<List<Domain.Entities.File>> GetFolderContentsAsync(Guid fileId);
        Task<List<Domain.Entities.File>> GetFolderContentsAsync(string name);
        Task<Domain.Entities.Config> GetConfigAsync();
        Task UpdateConfigAsync(Core.Domain.Entities.Fields field, bool value);
    }
}
