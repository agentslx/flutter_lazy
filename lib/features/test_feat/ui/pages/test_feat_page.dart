import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:formz/formz.dart';
import 'package:go_router/go_router.dart';

import '../../../../di.dart';
import '../../../../generated/colors.gen.dart';
import '../../../../widgets/app_containers/app_scaffold.dart';
import '../../../../widgets/app_containers/common_form_body.dart';
import '../../../../widgets/basic_components/app_buttons.dart';
import '../../../../widgets/basic_components/app_snackbar.dart';
import '../../../../widgets/basic_components/app_text_fields.dart';
import '../../cubits/test_feat_cubit/test_feat_cubit.dart';
import '../../data/models/test_feat_model.dart';
import '../widgets/test_feat_item_widget.dart';

class TestFeatPage extends StatelessWidget {
  const TestFeatPage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider<TestFeatCubit>(
      create: (_) => getIt<TestFeatCubit>()..initialize(),
      child: Builder(builder: (context) {
        return BlocConsumer<TestFeatCubit, TestFeatState>(
          listenWhen: (previous, current) => 
              previous.status != current.status || 
              previous.errorMessage != current.errorMessage,
          listener: (context, state) {
            // Handle error messages
            if (state.status == FormzSubmissionStatus.failure && state.errorMessage != null) {
              AppSnackbar.error(
                context: context,
                message: state.errorMessage ?? tr('common.unknown_error'),
              );
            }

            // Handle success messages
            if (state.status == FormzSubmissionStatus.success && state.successMessage != null) {
              AppSnackbar.success(
                context: context,
                message: state.successMessage!,
              );
            }
          },
          builder: (context, state) {
            return AppScaffold(
              appBar: AppBar(
                title: Text('test_feat.title'.tr()),
                actions: [
                  IconButton(
                    icon: const Icon(Icons.refresh),
                    onPressed: () => context.read<TestFeatCubit>().initialize(),
                  ),
                ],
              ),
              body: _buildBody(context, state),
              floatingActionButton: FloatingActionButton(
                onPressed: () => _showAddItemDialog(context),
                child: const Icon(Icons.add),
              ),
            );
          },
        );
      }),
    );
  }

  Widget _buildBody(BuildContext context, TestFeatState state) {
    // Show loading indicator when fetching data
    if (state.status == FormzSubmissionStatus.inProgress && state.items.isEmpty) {
      return const Center(child: CircularProgressIndicator());
    }

    // Show empty state when no data is available
    if (state.items.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(Icons.inbox, size: 64, color: ColorName.gray400),
            const SizedBox(height: 16),
            Text(
              'test_feat.empty_state'.tr(),
              style: Theme.of(context).textTheme.titleMedium,
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 24),
            AppPrimaryButton(
              label: 'test_feat.add_item'.tr(),
              onPressed: () => _showAddItemDialog(context),
              size: AppButtonSize.medium,
            ),
          ],
        ),
      );
    }

    // Show the data list
    return ListView.separated(
      padding: const EdgeInsets.all(16),
      itemCount: state.items.length,
      separatorBuilder: (context, index) => const SizedBox(height: 8),
      itemBuilder: (context, index) {
        final item = state.items[index];
        return TestFeatItemWidget(
          item: item,
          onTap: () => _showItemDetails(context, item),
          onEdit: () => _showEditItemDialog(context, item),
          onDelete: () => _confirmDelete(context, item),
        );
      },
    );
  }

  void _showItemDetails(BuildContext context, TestFeatModel item) {
    // Navigate to detail page or show a modal with item details
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (_) => Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(item.title, style: Theme.of(context).textTheme.titleLarge),
            const SizedBox(height: 8),
            Text(item.description),
            const SizedBox(height: 8),
            Text('Created: ${DateFormat.yMMMd().format(item.createdAt)}'),
            const SizedBox(height: 24),
            AppPrimaryButton(
              label: 'common.close'.tr(),
              onPressed: () => Navigator.pop(context),
              width: double.infinity,
            ),
          ],
        ),
      ),
    );
  }

  void _showAddItemDialog(BuildContext context) {
    final titleController = TextEditingController();
    final descriptionController = TextEditingController();

    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: Text('test_feat.add_item'.tr()),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            AppTextField(
              controller: titleController,
              label: 'test_feat.title_field'.tr(),
            ),
            const SizedBox(height: 16),
            AppTextField(
              controller: descriptionController,
              label: 'test_feat.description_field'.tr(),
              maxLines: 3,
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: Text('common.cancel'.tr()),
          ),
          ElevatedButton(
            onPressed: () {
              if (titleController.text.isNotEmpty) {
                context.read<TestFeatCubit>().createItem(
                  title: titleController.text,
                  description: descriptionController.text,
                );
                Navigator.pop(ctx);
              }
            },
            child: Text('common.save'.tr()),
          ),
        ],
      ),
    );
  }

  void _showEditItemDialog(BuildContext context, TestFeatModel item) {
    final titleController = TextEditingController(text: item.title);
    final descriptionController = TextEditingController(text: item.description);

    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: Text('test_feat.edit_item'.tr()),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            AppTextField(
              controller: titleController,
              label: 'test_feat.title_field'.tr(),
            ),
            const SizedBox(height: 16),
            AppTextField(
              controller: descriptionController,
              label: 'test_feat.description_field'.tr(),
              maxLines: 3,
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: Text('common.cancel'.tr()),
          ),
          ElevatedButton(
            onPressed: () {
              if (titleController.text.isNotEmpty) {
                context.read<TestFeatCubit>().updateItem(
                  id: item.id,
                  title: titleController.text,
                  description: descriptionController.text,
                );
                Navigator.pop(ctx);
              }
            },
            child: Text('common.save'.tr()),
          ),
        ],
      ),
    );
  }

  void _confirmDelete(BuildContext context, TestFeatModel item) {
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: Text('test_feat.delete_item'.tr()),
        content: Text('test_feat.delete_confirmation'.tr(args: [item.title])),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: Text('common.cancel'.tr()),
          ),
          ElevatedButton(
            onPressed: () {
              context.read<TestFeatCubit>().deleteItem(item.id);
              Navigator.pop(ctx);
            },
            style: ElevatedButton.styleFrom(
              foregroundColor: Colors.white, 
              backgroundColor: Colors.red,
            ),
            child: Text('common.delete'.tr()),
          ),
        ],
      ),
    );
  }
}
