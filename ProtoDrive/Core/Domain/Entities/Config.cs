using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Core.Domain.Entities
{
    public class Config
    {
        public required Guid UserId { get; set; }
        public string? Sorted { get; set; }
        public required bool Ascending { get; set; }
        public required bool CreatedAt { get; set; }
        public required bool EditedAt { get; set; }
        public required bool OwnedBy { get; set; }
        public required bool EditedBy { get; set; }
        public required bool Filtered { get; set; }
    }
    public enum Fields
    {
        Ascending,
        CreatedAt,
        EditedAt,
        OwnedBy,
        EditedBy,
        Filtered
    }
}
