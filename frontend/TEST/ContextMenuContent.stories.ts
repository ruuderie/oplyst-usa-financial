import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuContent from '../components/ui/context-menu/ContextMenuContent.vue';

const meta = {
  title: 'ContextMenuContent',
  component: ContextMenuContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuContent>;

export default meta;
type Story = StoryObj<typeof ContextMenuContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};