using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Core.Domain.Entities
{
    public class User
    {
        public required Guid Id { get; set; }
        public required string Login { get; set; }
        public required string Phc { get; set; }
    }
}
