using ProtoDrive.Core.Interfaces;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ProtoDrive.Core.Services
{
    public class AuthHeaderHandler : DelegatingHandler
    {
        private readonly ITokenStore _tokenStore;
        public AuthHeaderHandler(ITokenStore tokenStore)
        {
            _tokenStore = tokenStore;
        }
        protected override Task<HttpResponseMessage> SendAsync(
            HttpRequestMessage request,
            CancellationToken cancellationToken)
        {
            var token = _tokenStore.AccessToken;
            if (!string.IsNullOrEmpty(token) && request.Headers.Authorization == null)
            {
                request.Headers.Authorization = new System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", token);
            }
            return base.SendAsync(request, cancellationToken);
        }
    }
}
