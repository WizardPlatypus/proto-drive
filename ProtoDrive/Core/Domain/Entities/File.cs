using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Core.Domain.Entities
{
    public class File
    {
        public required Guid Id { get; set; }
        public required string Name { get; set; }
        public string? Path { get; set; }
        public required Guid OwnedBy { get; set; }
        public Guid? EditedBy { get; set; }
        public required DateTime CreatedAt { get; set; }
        public DateTime? EditedAt { get; set; }
    }
}
