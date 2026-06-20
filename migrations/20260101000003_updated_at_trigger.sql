-- BEFORE INSERT trigger on base tables
-- Auto-set updatedAt = NOW() if NULL (Prisma @updatedAt is app-level, not DB default)

CREATE OR REPLACE FUNCTION base_table_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW."updatedAt" IS NULL THEN
        NEW."updatedAt" = NOW();
    END IF;
    IF NEW."createdAt" IS NULL THEN
        NEW."createdAt" = NOW();
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply to all actual base tables (case-sensitive)
DO $$
DECLARE
    t TEXT;
    tables TEXT[] := ARRAY['Address', 'Article', 'Banner', 'Cart', 'Deposite',
                           'DepositeStatus', 'Landfills', 'Order', 'PaymentHistory',
                           'Product', 'ProductDelivery', 'Saller', 'Token', 'TrashType',
                           'User', 'UserDetail', 'VariantProduct'];
BEGIN
    FOREACH t IN ARRAY tables LOOP
        EXECUTE format('DROP TRIGGER IF EXISTS set_updated_at ON %I', t);
        EXECUTE format(
            'CREATE TRIGGER set_updated_at BEFORE INSERT ON %I FOR EACH ROW EXECUTE FUNCTION base_table_set_updated_at()',
            t
        );
    END LOOP;
END $$;
