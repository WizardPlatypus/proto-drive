using Core.Interfaces;
using Core.Services;
using Desktop.Services;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using System.Configuration;
using System.Data;
using System.Windows;
using ViewModels.Auth;
using ViewModels.FileExplorer;

namespace Desktop
{
    public partial class App : Application
    {
        private readonly IHost _host;

        public App()
        {
            _host = Host.CreateDefaultBuilder()
                .ConfigureServices((context, services) =>
                {
                    ConfigureServices(services);
                    ConfigureViewModels(services);
                    ConfigureViews(services);
                })
                .Build();
        }

        private void ConfigureServices(IServiceCollection services)
        {
            services.AddHttpClient<IApiService, ApiConsumer>(client =>
            {
                client.BaseAddress = new Uri("http://localhost:3001/");
            });
            services.AddSingleton<IDialogService, WpfDialogService>();
            services.AddSingleton<INavigationService, WpfNavigationService>();
        }

        private void ConfigureViewModels(IServiceCollection services)
        {
            services.AddTransient<LoginViewModel>();
            services.AddTransient<RegisterViewModel>();
            services.AddTransient<FileExplorerViewModel>();
        }

        private void ConfigureViews(IServiceCollection services)
        {
            services.AddSingleton<MainWindow>();
            services.AddTransient<Views.LoginPage>();
            services.AddTransient<Views.RegisterPage>();
            services.AddTransient<Views.FileExplorerPage>();
        }

        protected override async void OnStartup(StartupEventArgs e)
        {
            await _host.StartAsync();
            var mainWindow = _host.Services.GetRequiredService<MainWindow>();
            mainWindow.Show();
            base.OnStartup(e);
        }

        protected override async void OnExit(ExitEventArgs e)
        {
            using (_host)
            {
                await _host.StopAsync(TimeSpan.FromSeconds(5));
            }
            base.OnExit(e);
        }
    }
}
