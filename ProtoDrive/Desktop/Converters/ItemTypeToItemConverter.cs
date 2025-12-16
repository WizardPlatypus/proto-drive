using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Data;

namespace ProtoDrive.Desktop.Converters
{
    public class ItemTypeToItemConverter : IValueConverter
    {
        private const string BasePath = "pack://application:,,,/ProtoDrive.Desktop;component/Assets/";
        public object Convert(object value, Type targetType, object parameter, CultureInfo culture)
        {
            if (value is bool isFolder)
            {
                // Return the full pack URI path to the image resource
                if (isFolder)
                {
                    return new Uri(BasePath + "folder.png");
                }
                else
                {
                    // For simplicity, we just check if it's NOT a folder
                    return new Uri(BasePath + "file.png");
                }
            }
            return new Uri(BasePath + "file.png"); // Default icon
        }

        public object ConvertBack(object value, Type targetType, object parameter, CultureInfo culture)
        {
            throw new NotImplementedException();
        }
    }
}
