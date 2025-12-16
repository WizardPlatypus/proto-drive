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
        public Task RegisterAsync(string login, string password);
        public Task<string> LoginAsync(string login, string password);
        public Task UploadFileAsync(Stream file, string name, string destination);
        public Task<Stream> DownloadFileAsync(Guid fileId);
        public Task<List<Domain.Entities.File>> GetFolderContentsAsync(Guid fileId);
        public Task<List<Domain.Entities.File>> GetFolderContentsAsync(string name);
        public Task<Domain.Entities.Config> GetConfigAsync();
        public Task UpdateConfigAsync(Core.Domain.Entities.Fields field, bool value);
    }
}
