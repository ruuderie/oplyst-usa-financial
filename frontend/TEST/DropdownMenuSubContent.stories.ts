import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuSubContent from '../components/ui/dropdown-menu/DropdownMenuSubContent.vue';

const meta = {
  title: 'DropdownMenuSubContent',
  component: DropdownMenuSubContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuSubContent>;

export default meta;
type Story = StoryObj<typeof DropdownMenuSubContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};