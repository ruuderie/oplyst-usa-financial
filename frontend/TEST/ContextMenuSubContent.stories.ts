import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuSubContent from '../components/ui/context-menu/ContextMenuSubContent.vue';

const meta = {
  title: 'ContextMenuSubContent',
  component: ContextMenuSubContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuSubContent>;

export default meta;
type Story = StoryObj<typeof ContextMenuSubContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};