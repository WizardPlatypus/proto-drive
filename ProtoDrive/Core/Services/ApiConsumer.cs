using Core.Interfaces;
using Core.Domain.Entities;
using System.Net.Http.Json;
using System.Diagnostics;
using System.Net.Http.Headers;

namespace Core.Services
{
    public class ApiConsumer : IApiService
    {
        private readonly HttpClient _httpClient;
        private readonly string _baseUrl = "http://localhost:3001";
        public ApiConsumer(HttpClient httpClient)
        {
            _httpClient = httpClient;
            _httpClient.BaseAddress = new Uri(_baseUrl);
        }
        private void SetAuthorizationHeader(string token)
        {
            _httpClient.DefaultRequestHeaders.Authorization =
                new System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", token);
        }
        private record AuthResponse(string access_token);
        public Task<Stream> DownloadFileAsync(Guid fileId)
        {
            throw new NotImplementedException();
        }

        public async Task<Config> GetConfigAsync()
        {
            var response = await _httpClient.GetAsync("config");
            response.EnsureSuccessStatusCode();
            var config = await response.Content.ReadFromJsonAsync<Config>();
            if (config != null)
            {
                throw new InvalidOperationException("Failed to retrieve config from a successful response");
            }
            return config;
        }

        public Task<List<Domain.Entities.File>> GetFolderContentsAsync(Guid fileId)
        {
            throw new NotImplementedException();
        }

        public async Task<string> LoginAsync(string login, string password)
        {
            var loginRequest = new { login, password };
            var response = await _httpClient.PostAsJsonAsync("auth/login", loginRequest);
            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized ||
                response.StatusCode == System.Net.HttpStatusCode.BadRequest)
            {
                throw new UnauthorizedAccessException("Login failed. Check credentials.");
            }
            response.EnsureSuccessStatusCode();
            var authResult = await response.Content.ReadFromJsonAsync<AuthResponse>();
            if (authResult?.access_token == null)
            {
                throw new InvalidOperationException("Login endpoint returned an invalid token structure");
            }
            SetAuthorizationHeader(authResult.access_token);
            return authResult.access_token;
        }

        public async Task RegisterAsync(string login, string password)
        {
            var registerRequest = new { login, password };
            var response = await _httpClient.PostAsJsonAsync("auth/register", registerRequest);
            response.EnsureSuccessStatusCode();
        }

        private record ConfigUpdate
        {
            public required string field;
            public required bool value;
        }

        public async Task UpdateConfigAsync(Fields field, bool value)
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
            var response = await _httpClient.PutAsJsonAsync("config", request);
            response.EnsureSuccessStatusCode();
        }

        public async Task UploadFileAsync(Stream file, string name, string destination)
        {
            using var content = new MultipartFormDataContent();
            var streamContent = new StreamContent(file);
            streamContent.Headers.ContentType = new MediaTypeHeaderValue("application/octet-stream");
            content.Add(streamContent, name: "file", fileName: name);
            content.Add(new StringContent(destination), name: "destination");
            content.Add(new StringContent(name), name: "file_name");
            var response = await _httpClient.PostAsync("upload", content);
            response.EnsureSuccessStatusCode();
        }

        public void SetAccessToken(string accessToken)
        {
            SetAuthorizationHeader(accessToken);
        }
    }
}
