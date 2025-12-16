using ProtoDrive.Core.Interfaces;
using ProtoDrive.Core.Domain.Entities;
using System.Net.Http.Json;
using System.Diagnostics;
using System.Net.Http.Headers;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Data.Common;
using System.IO;
using System.Net.Http;

namespace ProtoDrive.Desktop.Services
{
    public class ApiConsumer : IApiService
    {
        private readonly HttpClient _apiClient;
        private readonly JsonSerializerOptions _options;
        private readonly ITokenStore _tokenStore;
        public ApiConsumer(HttpClient httpClient, JsonSerializerOptions options, ITokenStore tokenStore)
        {
            _apiClient = httpClient;
            _options = options;
            _tokenStore = tokenStore;
        }
        private record AuthOk
        {
            public required string AccessToken { get; set; }
        }
        Task<Stream> IApiService.DownloadFileAsync(Guid fileId)
        {
            throw new NotImplementedException();
        }

        async Task<Config> IApiService.GetConfigAsync()
        {
            var response = await _apiClient.GetAsync("config");
            response.EnsureSuccessStatusCode();
            var config = await response.Content.ReadFromJsonAsync<Config>(_options) ?? throw new InvalidOperationException("Failed to retrieve config from a successful response");
            return config;
        }

        async Task<List<Core.Domain.Entities.File>> IApiService.GetFolderContentsAsync(Guid fileId)
        {
            var response = await _apiClient.GetAsync($"folder/{fileId}");
            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
            {
                throw new InvalidOperationException("No such folder");
            }
            response.EnsureSuccessStatusCode();
            var files = await response.Content.ReadFromJsonAsync<List<Core.Domain.Entities.File>>(_options) ?? throw new InvalidOperationException("Failed to retrieve files from a successful response");
            return files;
        }

        async Task<List<Core.Domain.Entities.File>> IApiService.GetFolderContentsAsync(string name)
        {
            var response = await _apiClient.GetAsync($"folder?name={name}");
            response.EnsureSuccessStatusCode();
            var files = await response.Content.ReadFromJsonAsync<List<Core.Domain.Entities.File>>(_options) ?? throw new InvalidOperationException("Failed to retrieve files from a successful response");
            return files;
        }

        async Task<string> IApiService.LoginAsync(string login, string password)
        {
            var loginRequest = new { login, password };
            var response = await _apiClient.PostAsJsonAsync("auth/login", loginRequest, _options);
            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized ||
                response.StatusCode == System.Net.HttpStatusCode.BadRequest)
            {
                throw new UnauthorizedAccessException("Login failed. Check credentials.");
            }
            response.EnsureSuccessStatusCode();
            var authResult = await response.Content.ReadFromJsonAsync<AuthOk>(_options);
            string? token = authResult?.AccessToken;
            if (token == null)
            {
                throw new InvalidOperationException("Login endpoint returned an invalid token structure");
            }
            _tokenStore.AccessToken = token;
            _apiClient.DefaultRequestHeaders.Authorization = new AuthenticationHeaderValue("Bearer", token);
            return token!;
        }

        async Task IApiService.RegisterAsync(string login, string password)
        {
            var registerRequest = new { login, password };
            var response = await _apiClient.PostAsJsonAsync("auth/register", registerRequest, _options);
            if (response.StatusCode == System.Net.HttpStatusCode.Conflict)
            {
                var message = await response.Content.ReadAsStringAsync();
                throw new InvalidOperationException(message);
            }
            response.EnsureSuccessStatusCode();
        }

        private record ConfigUpdate
        {
            public required string field;
            public required bool value;
        }

        async Task IApiService.UpdateConfigAsync(Fields field, bool value)
        {
            string s = field switch
            {
                Fields.Ascending => "ascending",
                Fields.CreatedAt => "created_at",
                Fields.EditedAt => "edited_at",
                Fields.OwnedBy => "owned_by",
                Fields.EditedBy => "edited_by",
                Fields.Filtered => "filtered",
                _ => throw new UnreachableException()
            };
            var request = new ConfigUpdate { field = s, value = value };
            var response = await _apiClient.PutAsJsonAsync("config", request, _options);
            response.EnsureSuccessStatusCode();
        }

        async Task IApiService.UploadFileAsync(Stream file, string name, string destination)
        {
            using var content = new MultipartFormDataContent();
            var streamContent = new StreamContent(file);
            streamContent.Headers.ContentType = new MediaTypeHeaderValue("application/octet-stream");
            content.Add(streamContent, name: "file", fileName: name);
            content.Add(new StringContent(destination), name: "destination");
            content.Add(new StringContent(name), name: "file_name");
            var response = await _apiClient.PostAsync("upload", content);
            response.EnsureSuccessStatusCode();
        }
    }
}
