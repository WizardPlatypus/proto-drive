using ProtoDrive.Core.Interfaces;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ProtoDrive.Core.Services
{
    public class TokenStore : ITokenStore
    {
        public string? AccessToken { get; set; }
    }
}
