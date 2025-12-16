using CommunityToolkit.Mvvm.ComponentModel;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ProtoDrive.ViewModels.FileExplorer
{
    public class FileViewModel : ObservableObject
    {
        public Core.Domain.Entities.File File { get; }
        public FileViewModel(Core.Domain.Entities.File file)
        {
            File = file;
        }
        public bool IsFolder
        {
            get
            {
                return File.Path == null;
            }
        }
        public string Name => File.Name;
        public Guid Owner => File.OwnedBy;
        public Guid? Editor => File.EditedBy;
        public DateTime? Created => File.CreatedAt;
        public DateTime? LastModified => File.EditedAt;
    }
}
