using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Microsoft.Azure.Cosmos.Table;

namespace VariantMusicApi.Storage
{
    public class TableStore
    {
        private readonly CloudTable table;

        public TableStore(TableStoreConfig config)
        {
            if (CloudStorageAccount.TryParse(config.ConnectionString, out var account))
            {
                var tableClient = account.CreateCloudTableClient(new TableClientConfiguration());
                table = tableClient.GetTableReference(config.TableName);
                if (!table.Exists())
                {
                    throw new Exception("Specified table does not exist");
                }
            }
            else
            {
                throw new Exception("Invalid connection string");
            }
        }

        public Task<TableResult> Upsert(TableRow row)
        {
            var operation = TableOperation.InsertOrReplace(row);
            return table.ExecuteAsync(operation);
        }

        public IEnumerable<TableRow> Query()
        {
            return table.ExecuteQuery(new TableQuery<TableRow>());
        }
    }
}