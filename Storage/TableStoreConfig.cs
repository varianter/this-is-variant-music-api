using Microsoft.Extensions.Configuration;

namespace VariantMusicApi.Storage
{
    public class TableStoreConfig
    {
        public string ConnectionString { get; set; }
        public string TableName { get; set; }

        private const string configFileName = "storage.json";

        public static TableStoreConfig LoadTableStoreConfig()
        {
            IConfiguration config = new ConfigurationBuilder()
                .AddJsonFile(configFileName)
                .Build();

            return config.Get<TableStoreConfig>();
        }
    }
}